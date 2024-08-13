server:
    cargo run --release

upload-sample:
    curl "http://localhost:3000/properties/upload" -F file=@sample/japanese_properties.csv

download-sample:
    curl "http://localhost:3000/properties" > sample/japanese_properties.json

view-sample:
    jq . sample/japanese_properties.json

clean:
    rm sample/japanese_properties.json
