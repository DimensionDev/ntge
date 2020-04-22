import {NTGEKeypair, NTGEPublicKey, NTGESecretKey} from "ntge-core";

const pubkeyDiv = document.getElementById("pubkey");
const secrectkeyDiv = document.getElementById("secrectkey");

const createKeypairButton = document.getElementById("create_keypair");

createKeypairButton.addEventListener("click", event => {
    console.log("js::Creating keypair");
    const keypair = NTGEKeypair.new();
    const pubKey = keypair.get_public_key();
    const secretKey = keypair.get_secret_key();

    console.log("js::Serializing");
    pubkeyDiv.textContent = pubKey.serialize();
    secrectkeyDiv.textContent = secretKey.serialize();
});

