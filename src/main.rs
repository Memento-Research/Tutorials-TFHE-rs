use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8};

fn main() {
    // Todo proceso de TFHE tiene como principales etapas:
    // 1. Generar las claves de cifrado
    // 2. Enviar la clave pública al servidor
    // 3. Cifrar los datos
    // 4. Enviar los datos cifrados al servidor
    // 5. Realizar las operaciones en el servidor
    // 6. Recuperar los datos cifrados
    // 7. Descifrar los datos
    
    let config = ConfigBuilder::all_disabled().enable_default_uint8().build();

    let (client_key, server_key) = generate_keys(config);

    set_server_key(server_key);

    let clear_a = 27u8;
    let clear_b = 128u8;

    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);

    let result = a + b;

    let decrypted_result: u8 = result.decrypt(&client_key);

    let clear_result = clear_a + clear_b;

    assert_eq!(decrypted_result, clear_result);
}


