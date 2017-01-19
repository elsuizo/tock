#include <stdio.h>
#include <stdint.h>

#include <timer.h>
#include <rng.h>

uint8_t randbuf[256];

int main () {
  printf("[RNG] Test App\n");

  while (1) {
    rng_sync(randbuf, 256, 256);

    // Print the 256 bytes of randomness.
    char buf[600];
    int len = 600;
    len -= snprintf(buf, len, "Randomness: ");
    for (int i=0; i<256; i++) {
    	len -= snprintf(buf+(600-len), len, "%02x", randbuf[i]);
    }
    len -= snprintf(buf+(600-len), len, "\n\n");
    printf(buf);

    delay_ms(500);
  }
}
