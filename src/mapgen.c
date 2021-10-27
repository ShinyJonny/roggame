#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>

struct Header {
	uint32_t max_y;
	uint32_t max_x;
	uint32_t start_y;
	uint32_t start_x;
	uint32_t finish_y;
	uint32_t finish_x;
};

void write_header(FILE * file, struct Header *header);
void write_randombyte_map(FILE * file, long y, long x);
void write_numberfield_map(FILE * file, long y, long x);

int main(int argc, char *argv[])
{
	if (argc != 4) {
		fprintf(stderr, "%s: invalid argument count: %d", argv[0], argc - 1);
		return -1;
	}

	char file_path[256];
	strncpy(file_path, argv[1], 256);

	uint32_t y = atoi(argv[2]);

	if (y <= 0) {
		fprintf(stderr, "%s: invalid height: %s", argv[0], argv[2]);
		return -1;
	}

	uint32_t x = atoi(argv[3]);

	if (x <= 1) {
		fprintf(stderr, "%s: invalid width: %s", argv[0], argv[3]);
		return -1;
	}

	FILE *file = fopen(file_path, "wb");

	if (!file) {
		fprintf(stderr, "%s: could't open file: %s", argv[0], argv[1]);
		return -1;
	}

	fseek(file, 0, SEEK_SET);

	struct Header h;

	h.max_y = y,
	h.max_x = x,
	h.start_y = y / 2;
	h.start_x = x - 1;
	h.finish_y = y / 2;
	h.finish_x = 0;

	write_header(file, &h);
	write_numberfield_map(file, y, x);

	return 0;
}

void write_header(FILE * file, struct Header *header)
{
	fwrite((void *) &(header->max_y), 1, 4, file);
	fwrite((void *) &(header->max_x), 1, 4, file);
	fwrite((void *) &(header->start_y), 1, 4, file);
	fwrite((void *) &(header->start_x), 1, 4, file);
	fwrite((void *) &(header->finish_y), 1, 4, file);
	fwrite((void *) &(header->finish_x), 1, 4, file);

	return;
}

void write_randombyte_map(FILE * file, long y, long x)
{
	long byte;

	for (long i = 0; i < y * x; i++)
	{
		srand(clock());
		byte = (long) rand();

		fwrite((void *) &byte, 1, 1, file);
	}
}

void write_numberfield_map(FILE * file, long y, long x)
{
	for (long i = 0; i < y; i++)
	{
		for (long j = 0; j < x; j++)
		{
			fwrite((unsigned char *) &i, 1, 1, file);
		}
	}
}
