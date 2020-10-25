## Embed Link Generator
Embed Link Generator is an open source API which lets you generate embed links for sites such as Discord.

### API Documentation

#### GET /embed
Returns the HTML which includes metadata according to query parameters along with oEmbed data.

Name        | Type   | length                | Usage
------------|--------|-----------------------|-------------------------------
title       | string | up to 256 characters  | sets og:title in the metadata
description | string | up to 2048 characters | sets og:description in the metadata
site_name   | string | up to 2048 characters | sets og:site_name in the metadata
image       | url    | up to 2048 characters | sets og:image in the metadata
color       | hex    | exactly 6 characters  | sets og:theme_color in the metadata

### To do list

- [x] Add HTML generator endpoint
- [x] Add oEmbed generator endpoint
- [ ] Add API documentation
- [ ] Add index.html
- [ ] Add automatic tests
