#!/bin/bash

set -e

BINARY="./target/release/inspector_image"
IMAGE="../resources/image.jpeg"

echo "=== 🚀 Audit automatique du projet Inspector-Image ==="

echo ""
echo "1️⃣ Vérification du contenu du dépôt..."
if [ -f "README.md" ]; then
    echo "✅ README.md présent"
else
    echo "❌ README.md manquant"
fi

if [ -d "src" ]; then
    echo "✅ Code source présent"
else
    echo "❌ Dossier src/ manquant"
fi

echo ""
echo "2️⃣ Compilation du projet..."
cargo build --release
if [ -f "$BINARY" ]; then
    echo "✅ Binaire compilé : $BINARY"
else
    echo "❌ Erreur : binaire non généré"
    exit 1
fi

echo ""
echo "3️⃣ Test commande -map"
$BINARY map "$IMAGE" > map_output.txt
cat map_output.txt
if grep -q "32" map_output.txt && grep -q "34" map_output.txt; then
    echo "✅ Coordonnées GPS détectées (32, 34)"
else
    echo "⚠️ Vérifiez que les coordonnées correspondent bien (attendu : 32 / 34)"
fi

echo ""
echo "4️⃣ Test commande -steg"
$BINARY steg "$IMAGE" > steg_output.txt
head -n 5 steg_output.txt
if grep -q "-----BEGIN PGP PUBLIC KEY BLOCK-----" steg_output.txt && grep -q "-----END PGP PUBLIC KEY BLOCK-----" steg_output.txt; then
    echo "✅ Clé PGP détectée"
else
    echo "❌ Aucune clé PGP détectée"
fi

echo ""
echo "=== 🎯 Audit terminé ==="
#echo "Bonus à tester manuellement :"
#echo "  + Autres méthodes de stéganographie"
#echo "  + Interface graphique"
#echo "  + Comparaison d’images"
