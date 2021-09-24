# CC = gcc
# SRCS != fd ".cpp$" .
# OBJS := $(SRCS:.c=.o)

# TARGET := dfcore

# .PHONY: all clean

# all: $(TARGET)

# dfcore: $(OBJS)
# 	$(CC) -c $(CFLAGS) -o $@ $^

# clean:
# 	rm -f $(OBJS)

CC = gcc
CFLAGS = -g -Wall -std=c++14
LDFLAGS = -l dlapi -l m -l cfitsio -l tinyxml2 -l boost_system -l boost_filesystem
SRCS := $(wildcard *.cpp)
OBJDIR = .build
OBJS := $(SRCS:%.cpp=$(OBJDIR)/%.o)

DEPDIR = .dep
DEPS := $(SRCS:%.cpp=$(DEPDIR)/%.d)
DEPFLAGS = -MT $@ -MMD -MP -MF $(DEPDIR)/$*.d

.PHONY: clean
TARGET = dfcore

$(OBJDIR)/%.o: %.cpp | $(OBJDIR) $(DEPDIR)
	echo [CC] $@
	$(CC) -c $(CFLAGS) $(DEPFLAGS) -o $@ $<

$(TARGET): $(OBJS)
	echo [LD] $@
	$(CC) $(LDFLAGS) -o $@ $^

clean:
	rm -rf $(OBJDIR) $(DEPDIR) $(TARGET)

$(OBJDIR) $(DEPDIR):
	mkdir -p $@

$(DEPFILES):

include $(wildcard $(DEPFILES))
