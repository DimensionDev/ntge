#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Ed25519Keypair Ed25519Keypair;

typedef struct Ed25519PrivateKey Ed25519PrivateKey;

typedef struct Ed25519PublicKey Ed25519PublicKey;

Ed25519Keypair *c_ed25519_keypair_construct_from_private_key(const Ed25519PrivateKey *private_key);

void c_ed25519_keypair_destroy(Ed25519Keypair *keypair);

Ed25519PrivateKey *c_ed25519_keypair_get_private_key(Ed25519Keypair *keypair);

Ed25519PublicKey *c_ed25519_keypair_get_public_key(Ed25519Keypair *keypair);

Ed25519Keypair *c_ed25519_keypair_new(void);

Ed25519PrivateKey *c_ed25519_private_key_deserialize(const char *encoded);

void c_ed25519_private_key_destroy(Ed25519PrivateKey *private_key);

Ed25519PublicKey *c_ed25519_private_key_get_public_key(Ed25519PrivateKey *private_key);

Ed25519PrivateKey *c_ed25519_private_key_new(void);

char *c_ed25519_private_key_serialize(Ed25519PrivateKey *private_key);

Ed25519PublicKey *c_ed25519_public_key_deserialize(const char *encoded);

void c_ed25519_public_key_destroy(Ed25519PublicKey *public_key);

char *c_ed25519_public_key_serialize(Ed25519PublicKey *public_key);
