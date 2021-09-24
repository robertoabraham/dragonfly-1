CXX = g++
CXXFLAGS = -g -Wall
LIBS = -ldlapi -lm -ldl -lcfitsio -ltinyxml2 -lboost_system -lboost_filesystem -lcfitsio
SOURCES = $(wildcard *.cpp)
OBJS := $(SOURCES:.cpp=.o)
TARGET = dfcore

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CXX) $(LDFLAGS) $(CXXFLAGS) $^ -o $@ $(LIBS)

clean:
	rm -f $(OBJS)

install: $(TARGET)
	cp $(TARGET) /usr/bin

