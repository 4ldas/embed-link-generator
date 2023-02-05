## Embed Link Generator
Embed Link Generator is an open source API which lets you generate safe embed links for sites such as Discord.


## API Documentation

#### GET /embed
Returns the HTML which includes metadata according to query parameters along with oEmbed data.

##### Query parameters
Name        | Type   | Length                | Usage
------------|--------|-----------------------|-------------------------------
title       | string | up to 256 characters  | sets og:title in the metadata
description | string | up to 2048 characters | sets og:description in the metadata
site_name   | string | up to 2048 characters | sets og:site_name in the metadata
image       | url    | up to 2048 characters | sets og:image in the metadata
color       | hex    | exactly 6 characters  | sets og:theme_color in the metadata
##### this endpoint accepts all the query parameters of /oEmbed endpoint and then adds <link> tag to it inside of the in the html response.

#### GET /oEmbed
Returns the oEmbed json according to the query parameters.

##### Query parameters
Name         | Type        | Length                | Usage
-------------|-------------|-----------------------|------------------------------------
type         | oEmbed type | up to 256 characters  | sets embed type
author_name  | string      | up to 256 characters  | sets author/owner name
author_url   | url         | up to 2048 characters | sets author/owner url
provider_name| string      | up to 256 characters  | sets name for the resource provider
provider_url | url         | up to 2048 characters | sets url for the resource provider
