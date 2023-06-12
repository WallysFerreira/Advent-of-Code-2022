#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// Separate at newline
// Parse to int and add until newline
// Store the maximum

void readFile(char * path, char (*text)[6]) {
	FILE *fp = fopen(path, "r");
	char buf[2560][5];
	int line = 0;

	if (fp != NULL) {
		while (!feof(fp) && !ferror(fp)) {
			if (fgets(text[line], 6, fp) != NULL) {
				line++;
			}
		}
	} else {
		printf("Could not open file");
	}
	
	fclose(fp);
}

int main() {
	char text[2560][6];
	readFile("./test", text);

	for (int i = 0; i < 15; i++) {
		if (*text[i] != '\n') {
			printf("%s\n", text[i]);
		}
	}
	return 0;
}
