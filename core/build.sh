g++ -g -Wall $(ls |grep .cpp |tr "\n" ' ') -ldlapi -lm -ldl -lcfitsio -ltinyxml2 -lboost_system -lboost_filesystem -lcfitsio -o main
