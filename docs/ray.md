install ray in centos
```
   doc:
    https://docs.ray.io/en/latest/ray-overview/index.html

   install  venv：
     python -m venv .env  
     source .env/bin/activate

   change pip source：
     pip config set global.index-url https://pypi.tuna.tsinghua.edu.cn/simple
   
   install ray：
     pip install -U "ray[default]"
   
   run ray:
     ray start --head --dashboard-host='0.0.0.0' --dashboard-port=8265  #header node
     ray start --address='192.168.0.190:6379'    #worker node
     ray status      #watch status
```

first use ray by python
```
  code：
    import ray
    ray.init()

    @ray.remote
    def f(x):
        return x * x

    futures = [f.remote(i) for i in range(4)]
    print(ray.get(futures)) # [0, 1, 4, 9]
    
   tips：
    1.import ray and ray.init() to use it
    2.annotation @ray.remote means i wanna run this function remote
    3.remote() to real use it 
    4.ray.get() is  fetch the result
```

realize plasma 
```
   doc：
    https://arrow.apache.org/docs/python/plasma.html#the-plasma-in-memory-object-store
    
   define：
    In-Memory Distributed Object Store
    every node exists a Object Store,then realize zero-copy data sharing by shared-memory in node
```
  
    
   










 
 
