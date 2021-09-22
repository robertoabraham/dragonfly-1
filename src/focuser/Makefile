CC = clang
CFLAGS = -std=c99 -g
FFLAGS = 
MANAGER?=homebrew
ifeq ($(MANAGER),homebrew)
        LFLAGS = -L /usr/local/lib -l cfitsio
        INCDIR = /usr/local/include
else
        LFLAGS = -L /opt/local/lib -l cfitsio
        INCDIR = /opt/local/include
endif

DEPS = 
PROGRAMS = birger

%.o: %.c $(DEPS)
	$(CC) -c $(CFLAGS) -I${INCDIR} -o $@ $< 

all: birger
birger: birger.o 
	$(CC) -o $@ $^ $(FFLAGS) ${LFLAGS}

clean:
	rm -f *.o
	rm $(PROGRAMS)

install: $(PROGRAMS)
	mv $(PROGRAMS) /usr/local/bin
