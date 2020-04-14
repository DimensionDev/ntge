#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Ed25519Keypair Ed25519Keypair;

typedef struct Item Item;

Ed25519Keypair *c_new_ed25519_keypair(void);

void item_destroy(Item *item);

Item *item_new(void);
