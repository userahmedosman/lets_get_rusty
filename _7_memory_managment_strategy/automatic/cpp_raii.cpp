#include <iostream>
#include <memory>


using namespace std;

class Car{
    public:
    void do_something(){
        cout << "This is something method" << endl;
    }
};

class CarManager{
    private:
    Car* car;

    public:
    CarManager(Car* c): car(c) {}

    
    
    ~CarManager(){
        // delete allocation on object destruction
        delete car;
    }
    
};

bool handle_dummy_issue(int num){
 cout << "this handles dummy issue that may fail !" << endl;
 return num > 1 ? true : false;
}

void raii_traditional(){
        CarManager car = CarManager(new Car); // memory safe initialization
        if(handle_dummy_issue(0)) return;
}
void raii_stl(){
    unique_ptr<Car> car = make_unique<Car>(); // modern stl based memory safe initialization
    unique_ptr<Car> car2 = move(car); // move owner ship
    shared_ptr<Car> shared = make_shared<Car>(); // to share ownership
    shared_ptr<Car> shared2 = shared;
    if(handle_dummy_issue(0)) return;
}
int main(){
    raii_traditional();
    return 0;
}