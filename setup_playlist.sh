#!/bin/sh

PROJ_NAME=2017show

# Remove old playlist
rm output/playlists/$PROJ_NAME.json

# Put blank playlist file in its place
echo "{\"name\":\""$PROJ_NAME"\",\"items\":[]}" > output/playlists/$PROJ_NAME.json

# Create playlist
proton_runner add-playlist-item $PROJ_NAME 0 --music=Music/Announce_1PreShow.ogg
proton_runner add-playlist-item $PROJ_NAME 1 --dur=1000
proton_runner add-playlist-item $PROJ_NAME 2 --seq=output/1_Aurora.json --music=Music/1_Aurora.ogg
proton_runner add-playlist-item $PROJ_NAME 3 --dur=1000
proton_runner add-playlist-item $PROJ_NAME 4 --seq=output/2_Collide.json --music=Music/2_Collide.ogg
proton_runner add-playlist-item $PROJ_NAME 5 --dur=1000
proton_runner add-playlist-item $PROJ_NAME 6 --seq=output/3_GalaxyGroove.json --music=Music/3_GalaxyGroove.ogg
proton_runner add-playlist-item $PROJ_NAME 7 --dur=1000
proton_runner add-playlist-item $PROJ_NAME 8 --music=Music/Announce_2MidShow.ogg
proton_runner add-playlist-item $PROJ_NAME 9 --dur=1000
proton_runner add-playlist-item $PROJ_NAME 10 --seq=output/4_DnD.json --music=Music/4_DNDfinal.ogg
proton_runner add-playlist-item $PROJ_NAME 11 --dur=1000
proton_runner add-playlist-item $PROJ_NAME 12 --seq=output/5_GlorytotheBells.json --music=Music/5_GlorytotheBells.ogg
proton_runner add-playlist-item $PROJ_NAME 13 --dur=1000
proton_runner add-playlist-item $PROJ_NAME 14 --music=Music/Announce_3PostShow.ogg
proton_runner add-playlist-item $PROJ_NAME 15 --music=Music/Announce_1PremiereParty.ogg
proton_runner add-playlist-item $PROJ_NAME 16 --dur=1000
