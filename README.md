# key value extractor

Extract key/value pairs from a input path depending on a pattern.

    %artist%-%title%

will extract a hashmap with:

    artist: Caravan Palace
    title: Clash

from both Caravan Palace-Clash.mp3 and ~/videos/swing/Caravan Palace-Clash.mp4

Subfolders can also be used, like the following will do as expected.

    %artist%/albums/%album%/%title%

    La Dispute/albums/Wildlife/King Park.mp3
