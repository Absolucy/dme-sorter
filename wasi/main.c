#include "dme-sorter.h"
#include "w2c2_base.h"
#include "wasi/wasi.h"
#include <stdio.h>

void trap(Trap trap) {
	fprintf(stderr, "TRAP: %s\n", trapDescription(trap));
	abort();
}

wasmMemory* wasiMemory(void* instance) {
	return dmesorter_memory((dmesorterInstance*)instance);
}

extern char** environ;

int main(int argc, char* argv[]) {
	if (!wasiInit(argc, argv, environ)) {
		fprintf(stderr, "failed to init WASI\n");
		return 1;
	}

	if (!wasiFileDescriptorAdd(-1, "/", NULL)) {
		fprintf(stderr, "failed to add preopen\n");
		return 1;
	}

	{
		dmesorterInstance instance;
		dmesorterInstantiate(&instance, NULL);
		dmesorter__start(&instance);
		dmesorterFreeInstance(&instance);
	}

	return 0;
}
