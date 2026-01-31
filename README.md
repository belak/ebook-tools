# ebook-tools

This repository provides an opinionated set of libraries, command-line tools,
and a GUI intended to replace Calibre for simple use-cases.

## Command Line Tools

- [ ] `ebook-info` - debugging tool to extract information from an ebook
- [ ] `ebook-edit` - tool for making simple edits to ebooks
- [ ] `ebook-convert` - tool to convert an ebook between formats
- [ ] `ebook-sync` - sync a folder or collection of ebooks to a target device
- [ ] `ebook-drm` - tool for cleaning DRM from ebooks

## Format Support

For general book support, we aim to handle the most common formats.

- [ ] epub
- [ ] kepub
- [ ] mobi
- [ ] azw3

For DRM, this focuses on cleaning ebooks which have been purchased.

- [ ] acsm (for converting to epub)
- [ ] acsm (for converting to pdf)
- [ ] kobo-protected epub

## Edit Operations

- [ ] Modify metadata
- [ ] Modify cover

## Device Support

In order to be included, all devices must be actively supported by a developer.
My time is limited, so I will only personally support devices I have and use,
but I am happy to include support for other devices if pull requests and
ongoing support is provided.

- Kobo Clara BW (@belak)
- Xteink X4 (@belak)
