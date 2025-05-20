# mkghal
Creates "ghost albums" for use as placeholders in local files storage in apps like Spotify. Running `mkghal` will query you for album/artist name, and a url for the cover art, and create the placeholder in your current directory.

```
Usage: mkghal [OPTIONS]

Options:
  -b, --bandcamp <BANDCAMP_LINK>  Bandcamp album URL
  -h, --help                      Print help
```

## Planned additions (TODO)
- last.fm: pass a last.fm link to create a "ghost album" out of the data
- RateYourMusic: currently infeasible, RYM is anti-webscraping. if there's enough interest, I'll look into applying for API access
