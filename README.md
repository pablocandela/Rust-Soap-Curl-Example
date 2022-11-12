# Rust-Soap-Curl-Example

Web Service publico alojado en https://www.dataaccess.com/webservicesserver/NumberConversion.wso

Petición curl realizada:

curl --location --request POST 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso' \
     --header 'Content-Type: text/xml; charset=utf-8' \
    --data-raw '<?xml version="1.0" encoding="utf-8"?>
    <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
        <soap:Body>
            <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
                <ubiNum>500</ubiNum>
            </NumberToWords>
        </soap:Body>
    </soap:Envelope>'
