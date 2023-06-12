#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// Separate at newline
// Parse to int and add until newline
// Store the maximum

void readFile(char * path, char (*text)[6]) {
	FILE *fp = fopen(path, "r");
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
	int max = 0;
	int sum = 0;

	for (int i = 0; i < 15; i++) {

		if (*text[i] != '\n') {
			sum += atoi(text[i]);
		} else {
			if (sum >= max) {
				max = sum;
			}

			sum = 0;
		}
	}

	printf("%d", max);
	return 0;
}
