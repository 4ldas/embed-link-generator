## Embed Link Generator
Embed Link Generator is an open source API which lets you generate embed links for sites such as Discord.

### API Documentation

#### GET /embed
Returns the HTML which includes metadata according to query parameters along with oEmbed data.

Name        | Type   | length                | Usage
------------|--------|-----------------------|------
title       | string | up to 256 characters  |
description | string | up to 2048 characters |
site_name   | string | up to 2048 characters |
image       | url    | up to 2048 characters |
color       | hex    | exactly 6 characters  |

### To do list

- [x] Add HTML generator endpoint
- [x] Add oEmbed generator endpoint
- [ ] Add API documentation
- [ ] Add index.html
- [ ] Add automatic tests
