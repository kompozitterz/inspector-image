#!/bin/bash
# Génère une image JPEG blanche avec des coordonnées GPS EXIF

# Nom du fichier (par défaut : test.jpg)
FILENAME=${1:-test.jpg}

# Coordonnées (par défaut : Paris 48.8566, 2.3522)
LAT=${2:-48.8566}
LON=${3:-2.3522}
LATREF="N"
LONREF="E"

# Si la longitude est négative, mets la référence à Ouest
if (( $(echo "$LON < 0" | bc -l) )); then
  LONREF="W"
  LON=$(echo "$LON * -1" | bc -l)
fi

# Si la latitude est négative, mets la référence à Sud
if (( $(echo "$LAT < 0" | bc -l) )); then
  LATREF="S"
  LAT=$(echo "$LAT * -1" | bc -l)
fi

echo "➡️  Génération de l'image $FILENAME avec GPS ($LAT $LATREF, $LON $LONREF)"

# 1. Crée une image blanche 500x500 avec ImageMagick
convert -size 500x500 xc:white "$FILENAME"

# 2. Ajoute les coordonnées GPS avec exiftool
exiftool -overwrite_original \
  -GPSLatitude="$LAT" -GPSLatitudeRef=$LATREF \
  -GPSLongitude="$LON" -GPSLongitudeRef=$LONREF \
  "$FILENAME"

echo "✅ Image générée : $FILENAME"
