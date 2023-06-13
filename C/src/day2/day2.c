#include "day2.h"
#include <stdio.h>
#define LINE_LENGTH 2

int main() {
	FILE *fp = fopen("./test", "r");
	char line[LINE_LENGTH];

	while (fgets(line, LINE_LENGTH, fp) != NULL) {
		if (line[0] != ' ') {
			if (line[0] == '\n') {
				printf("Quebrou a linha\n");
			}
			printf("%s", line);
		}
	}
}
