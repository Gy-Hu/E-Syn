INC=-I./include

all: aiglec aigviz

aiglec: src/aiglec.cpp src/aigreader.cpp src/structures.cpp
	g++ $(INC) -O3 src/aiglec.cpp src/aigreader.cpp src/structures.cpp -o aiglec

aigviz: src/aigviz.cpp src/aigreader.cpp src/structures.cpp
	g++ $(INC) -O3 src/aigviz.cpp src/aigreader.cpp src/structures.cpp -o aigviz

clean:
	rm -rf *.png

distclean:
	rm -rf aiglec aigviz