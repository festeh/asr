#!/bin/bash

echo "Uploading $1, lang: $2"
base64 $1 > audio.b64
echo -n '{"audio": "' > audio.json
cat audio.b64 >> audio.json
echo -n '", "lang": "' >> audio.json
echo -n $2 >> audio.json
echo '"}' >> audio.json
curl -X POST -H "Content-Type: application/json" --data @audio.json http://localhost:8080/recognize

