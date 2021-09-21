g++ -g -Wall -ldlapi -lm -ldl -lcfitsio -ltinyxml2 -lboost_system -lboost_filesystem -lcfitsio $(ls |grep .cpp |tr "\n" ' ') -o main
