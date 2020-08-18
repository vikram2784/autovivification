[![Build Status](https://cloud.drone.io/api/badges/cotigao/autovivification/status.svg)](https://cloud.drone.io/cotigao/autovivification)

## AutoVivification

Autovivification is the concept that a hash style data structure can make inferences about its internal structure as it is being created. For instance, creating a nested hash under one of the keys would normally require you to explicitly state that you wanted to create a hash there. Autovivification allows you to skip this step and create the nested structure on the fly.


Eg. a.b.c.d.e = 1  b,c,d,e get created on the fly, if not present
