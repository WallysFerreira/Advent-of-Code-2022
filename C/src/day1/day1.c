#include "day1.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// Separate at newline
// Parse to int and add until newline
// Store the maximum

void read_file(char * path, char (*text)[6], int * lines) {
	FILE *fp = fopen(path, "r");
	int line = 0;

	if (fp != NULL) {
		while (!feof(fp) && !ferror(fp)) {
			if (fgets(text[line], 6, fp) != NULL) {
				line++;
			}
		}

		if (ferror(fp)) {
			printf("Algum erro");
		}

		if (feof(fp)) {
			printf("Fim do arquivo");
		}
	} else {
		printf("Could not open file");
	}

	printf("%d\n", line);
	*lines = line;
	fclose(fp);
}

int sum_calories(char * path) {
	char text[2560][6];
	int * lines = 0;
	read_file(path, text, lines);
	int max = 0;
	int sum = 0;

	for (int i = 0; i < *lines; i++) {

		if (*text[i] != '\n') {
			sum += atoi(text[i]);
		} else {
			if (sum >= max) {
				max = sum;
			}

			sum = 0;
		}
	}

	return max;
}

int main() {
	printf("%d\n", sum_calories("./input"));
}
