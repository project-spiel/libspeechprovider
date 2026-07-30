Title: Overview

# Overview

A speech provider synthesizes speech to audio and pipes the audio data back to
the caller through a file descriptor handed to it in the
`org.freedesktop.Speech.Provider.Synthesize()` call. Each voice advertises the
format it produces so that the receiver can play or transcode it correctly. For
example, `audio/x-raw,format=S16LE,channels=1,rate=22050` means uncompressed PCM
in signed 16 bit little-endian byte order, one channel, at 22.05 kHz.

Plain audio, however, carries no information about *what* is being spoken at any
given moment. Screen readers and read-aloud interfaces need to highlight the
word or sentence that is currently audible, which means the stream has to say
where each word begins relative to the audio around it.

This library defines a binary stream format that interleaves such metadata with
the audio, so that a client can map a portion of the audio back onto a range of
the input text. [class@SpeechProvider.StreamWriter] produces the format and
[class@SpeechProvider.StreamReader] consumes it; a provider that only ever emits
plain audio does not need either, and can write raw bytes to the pipe.

# Advertising the Format

A voice that emits this format advertises its output format with the
`audio/x-spiel` media type instead of `audio/x-raw`, keeping the same
parameters. For 16 bit PCM at 22.05 kHz the full format specifier is:

    audio/x-spiel,format=S16LE,channels=1,rate=22050

The parameters describe the audio *payload* carried inside the stream's audio
chunks, not the stream as a whole. A client that decodes the chunk framing is
left with audio matching the same specifier with `audio/x-raw` substituted back
in.

A client that encounters `audio/x-spiel` knows the stream is not raw audio and
that it must be de-framed before playback. A client that does not understand the
media type should refrain from playing the stream at all rather than treat it as
raw audio, since the header and chunk framing would be audible as noise.

Advertising `audio/x-spiel` says the stream *can* carry
events; which events a voice actually emits is advertised separately through the
`EVENTS_*` bits of the voice's [flags@SpeechProvider.VoiceFeature] field. A
voice should not claim an `EVENTS_*` feature unless it advertises
`audio/x-spiel`, since there would be no way to deliver those events.

# Stream Structure

    ┌──────────────┐
    │ 4-byte header│  "0.01"
    ├──────────────┤
    │ chunk        │  audio or event
    ├──────────────┤
    │ chunk        │
    │      ...     │
    └──────────────┘
       EOF = end of utterance

The stream opens with a fixed 4 byte header that carries the format version.
Everything after it is a sequence of self-delimiting chunks, each tagged with a
one byte type, sent until the writer closes the pipe.

Chunk order is unconstrained: an audio chunk may follow an audio chunk, an event
chunk may follow an event chunk, and a stream may legitimately contain no chunks
at all (for empty input text). Readers must therefore be prepared for any
ordering and must not assume that events and audio alternate.

All multi-byte integers are written in **host byte order** with no padding
between fields. The stream is only ever passed between processes on the same
machine over a file descriptor, so both ends necessarily agree; implementations
that hand-roll the format must use native, unaligned packing rather than network
byte order.

# Stream Header

| Offset | Size | Field     | Value                            |
| -----: | ---: | --------- | -------------------------------- |
|      0 |    4 | `version` | ASCII version string, e.g. `0.01` |

The header is the four ASCII characters of the format version, *not*
NUL-terminated. The current version is `0.01`.

The reader compares the four bytes against the version it was built for and
reports the mismatch to its caller, which keeps a reader from misinterpreting a
stream written by an incompatible writer as chunk data. There is no negotiation:
[method@SpeechProvider.StreamReader.get_stream_header] returns `FALSE` on
mismatch, and the only reasonable response is to abandon the stream. Note that
the four bytes have been consumed either way.

# Audio Chunk

An audio chunk carries a run of encoded audio in the format the voice
advertised.

| Offset | Size | Field        | Value                                     |
| -----: | ---: | ------------ | ----------------------------------------- |
|      0 |    1 | `chunk_type` | `SPEECH_PROVIDER_CHUNK_TYPE_AUDIO` (`1`)  |
|      1 |    4 | `length`     | Number of audio bytes that follow         |
|      5 | *n*  | `data`       | *n* = `length` bytes of audio             |

Chunk boundaries carry no meaning beyond framing. A provider may split its
output into as many or as few chunks as is convenient, and a reader must not
infer anything from where the splits fall — in particular, a chunk is not
guaranteed to hold a whole number of audio frames, so a consumer that needs
frame alignment must buffer across chunks. In practice providers split audio
precisely where an event needs to be inserted (see below).

Zero-length audio chunks are legal but carry no information.

# Event Chunk

An event chunk marks a landmark in the text that is about to be spoken — a word
or sentence boundary, or an SSML mark.

| Offset | Size | Field              | Value                                       |
| -----: | ---: | ------------------ | ------------------------------------------- |
|      0 |    1 | `chunk_type`       | `SPEECH_PROVIDER_CHUNK_TYPE_EVENT` (`2`)    |
|      1 |    1 | `event_type`       | A [enum@SpeechProvider.EventType] value     |
|      2 |    4 | `range_start`      | Start offset in the input text              |
|      6 |    4 | `range_end`        | End offset in the input text                |
|     10 |    4 | `mark_name_length` | Byte length of the mark name that follows   |
|     14 | *n*  | `mark_name`        | *n* = `mark_name_length` bytes of UTF-8     |

The fixed part of an event chunk is therefore always 14 bytes.

`range_start` and `range_end` are character offsets into the text that was
passed to `Synthesize()`, forming the half-open range `[range_start,
range_end)`.

`mark_name` is a UTF-8 string that is **not** NUL-terminated, and its length
field may be zero. It is only meaningful for `MARK` events, which correspond to
an SSML `<mark name="foo"/>` element in the input; for every other event type
providers send an empty name.

## Event Types

| Value | Name       | Meaning                                                         |
| ----: | ---------- | --------------------------------------------------------------- |
|     0 | `NONE`     | Not a valid event on the wire; reported by the reader when no event was read |
|     1 | `WORD`     | A word is about to be spoken                                    |
|     2 | `SENTENCE` | A sentence is about to be spoken                                |
|     3 | `RANGE`    | An unspecified range is about to be spoken                      |
|     4 | `MARK`     | An SSML mark has been reached                                    |

## Positioning Events

An event applies to the audio that *follows* it: the writer emits the event
immediately before the first audio chunk containing the range it describes.
This is what makes the format worth having — the position of the event in the
byte stream, rather than any timestamp, is what ties text to audio.

A provider does this by splitting its audio at the sample where a landmark
begins:

1. Write the audio up to the landmark as one or more audio chunks.
2. Write the event chunk.
3. Continue writing audio.

Synthesizers usually report the audio position of an event in milliseconds, so
the provider converts that to a byte offset in its own output —
`ms * rate / 1000 * bytes_per_frame` for PCM — and tracks how many bytes it has
already written in order to find the split point within the current buffer.

Because a client typically observes the stream as it plays, this ordering means
an event is seen slightly *before* the corresponding audio is heard, by however
much audio the client has buffered.

# Reading a Stream

The reader is a pull API with one byte of lookahead. Each getter peeks at the
next chunk's type byte, and if the chunk is not of the requested kind it returns
`FALSE` without consuming anything, leaving the chunk available to the other
getter. Calling the "wrong" getter is therefore harmless, and the idiomatic loop
simply tries both:

```c
SpeechProviderStreamReader *reader = speech_provider_stream_reader_new (fd);

if (!speech_provider_stream_reader_get_stream_header (reader))
  return; /* version mismatch: not a stream we can read */

for (;;)
  {
    SpeechProviderEventType event_type = SPEECH_PROVIDER_EVENT_TYPE_NONE;
    guint32 range_start = 0, range_end = 0;
    g_autofree char *mark_name = NULL;
    guint8 *chunk = NULL;
    guint32 chunk_size = 0;
    gboolean got_event, got_audio;

    got_event = speech_provider_stream_reader_get_event (
        reader, &event_type, &range_start, &range_end, &mark_name);
    if (got_event)
      handle_event (event_type, range_start, range_end, mark_name);

    got_audio =
        speech_provider_stream_reader_get_audio (reader, &chunk, &chunk_size);
    if (got_audio)
      handle_audio (chunk, chunk_size); /* takes ownership of chunk */

    if (!got_event && !got_audio)
      break; /* end of stream */
  }

speech_provider_stream_reader_close (reader);
```

[method@SpeechProvider.StreamReader.get_stream_header] must be called exactly
once, before any chunk is read.

Note that both getters returning `FALSE` is the only end-of-stream signal, since
the format has no terminator chunk: when the writer closes the pipe, reads
return no data and no chunk type can be determined. On a non-blocking file
descriptor this is indistinguishable from "no data has arrived yet", so a reader
using non-blocking I/O should poll for readability before entering the loop, as
GStreamer's `spielprovidersrc` element in `libspiel` does.

On a successful read, [method@SpeechProvider.StreamReader.get_audio] hands over
a newly allocated buffer that the caller owns; on failure it reports a size of
zero. [method@SpeechProvider.StreamReader.get_event] reports an event type of
`NONE` on failure, and yields a `NULL` mark name whenever the mark name length
was zero.

The reader does not close its file descriptor when it is finalized; call
[method@SpeechProvider.StreamReader.close] when you are done with the stream.

# Writing a Stream

A provider creates a writer around the file descriptor it received from
`Synthesize()`, sends the header once, and then interleaves audio and events as
synthesis proceeds:

```c
SpeechProviderStreamWriter *writer = speech_provider_stream_writer_new (fd);

speech_provider_stream_writer_send_stream_header (writer);

speech_provider_stream_writer_send_audio (writer, leading_audio, leading_size);
speech_provider_stream_writer_send_event (writer, SPEECH_PROVIDER_EVENT_TYPE_WORD,
                                          0, 5, "");
speech_provider_stream_writer_send_audio (writer, word_audio, word_size);

speech_provider_stream_writer_close (writer);
```

[method@SpeechProvider.StreamWriter.send_stream_header] must be called exactly
once, before any chunk is written.
[method@SpeechProvider.StreamWriter.send_event] requires a non-`NULL` mark name,
so pass an empty string for events that do not carry one. Closing the pipe is
what tells the client the utterance is complete, and the writer closes its file
descriptor when it is finalized as well as on
[method@SpeechProvider.StreamWriter.close].

Providers are expected to serve concurrent `Synthesize()` calls, which means one
writer per call. Interleaving writes from two writers on the *same* file
descriptor would corrupt the framing, so each call must get its own pipe.

# Worked Example

A minimal stream containing four bytes of audio, a word event covering the first
five characters of the input text, and four more bytes of audio:

```
30 2E 30 31                          "0.01"          stream header

01                                   chunk type      audio
04 00 00 00                          length          4
11 22 33 44                          audio data

02                                   chunk type      event
01                                   event type      WORD
00 00 00 00                          range start     0
05 00 00 00                          range end       5
00 00 00 00                          mark name len   0

01                                   chunk type      audio
04 00 00 00                          length          4
55 66 77 88                          audio data
```

Integer fields are shown little-endian, as they would appear on a
little-endian host.

# Notes for Implementers

The framing is deliberately simple enough to write by hand — a provider in a
language without GObject bindings can emit it with a handful of `struct.pack`
calls — but a few details are easy to get wrong:

- **Pack, do not align.** The header, chunk type, and event structures are
  packed; in particular an event chunk is 14 bytes, not 16. A naive struct
  definition in a language that aligns `guint32` fields will insert padding
  after `event_type` and desynchronize the stream.
- **Use host byte order**, as described under "Stream Structure" above.
- **Do short reads and writes properly.** A single `write()` of a large audio
  chunk to a pipe can transfer fewer bytes than requested, and a `read()` can
  likewise return a partial chunk; loop until the whole field or payload has
  been transferred. This applies to the C implementation in this library as
  well, which is why callers should keep individual audio chunks modest in size.
- **`mark_name_length` is a byte count**, not a character count, since that is
  what the reader consumes.

# Versioning

The version in the header exists so that a reader can refuse a stream it cannot
parse rather than misinterpret it. Any change to the framing — a new field, a
different integer width, a new chunk type that older readers could not skip —
requires a version bump.

New [enum@SpeechProvider.EventType] values do not, because an unknown event type
still occupies a well-formed 14 byte chunk: readers can skip past what they do
not recognize, so readers should ignore unfamiliar event types rather than treat
them as errors. New [enum@SpeechProvider.ChunkType] values, by contrast, are not
skippable, since a reader that does not know a chunk's type cannot know its
length.
