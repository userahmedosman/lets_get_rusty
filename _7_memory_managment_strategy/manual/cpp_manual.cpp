#include <iostream>

using namespace std;
class Car {
    
    public:
    Car(){

    }
    void do_something(){
        cout << "This is something method" << endl;
    }
};



bool handle_dummy_issue(int num){
 cout << "this handles dummy issue that may fail !" << endl;
 return num > 1 ? true : false;
}

void manual_memory_managment(){
    Car* car = new Car();
    
    if(handle_dummy_issue(1)) return; // this early return may lead to memory 
    delete car;
}



int main(){
  
    manual_memory_managment();
    return 0;
}