use std::process::Command;

fn main() {
    let body: String  = String::from("<?xml version='1.0' encoding='utf-8'?> 
                                        <soap:Envelope xmlns:soap='http://schemas.xmlsoap.org/soap/envelope/'> 
                                            <soap:Body>
                                                <NumberToWords xmlns='http://www.dataaccess.com/webservicesserver/'>
                                                    <ubiNum>500</ubiNum>
                                                </NumberToWords>
                                            </soap:Body> 
                                        </soap:Envelope>");
    let output = Command::new("curl")
        .args(&[
            "-X",
            "POST",
            "--data-raw",
            &body,
            "-H",
            "Content-Type: text/xml; charset=utf-8",
            "https://www.dataaccess.com/webservicesserver/NumberConversion.wso",
        ])
        .output()
        .unwrap_or_else(|e| panic!("Fallo al ejecutar la petición: {}", e));

    if output.status.success() {
        let output = String::from_utf8_lossy(&output.stdout);

        print!("Curl exitoso, la salida es:\n{}", output);
    } else {
        let output = String::from_utf8_lossy(&output.stderr);

        print!("Curl fallido, la salida es: \n{}", output);
    }
}