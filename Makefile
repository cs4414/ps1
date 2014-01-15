PROGRAM_NAME = zhttpto

all: $(PROGRAM_NAME)

$(PROGRAM_NAME): 
	rustc $(PROGRAM_NAME).rs

clean :
	$(RM) $(PROGRAM_NAME)
    
run: ${PROGRAM_NAME}
	./${PROGRAM_NAME}
