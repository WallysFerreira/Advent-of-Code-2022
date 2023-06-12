#include <stdio.h>
#include <string.h>
#include <stdlib.h>

void readFile(char * path, char * text) {
	FILE *fp = fopen(path, "r");
	char buf[2560];

	if (fp != NULL) {
		while(fgets(buf, 2560, fp)) {
			strcat(text, buf);
		}
	} else {
		printf("Could not open file");
	}
	
	fclose(fp);
}

int main() {
	char text[2560];

	readFile("./test", text);
	printf("%s", text);
	return 0;
}
