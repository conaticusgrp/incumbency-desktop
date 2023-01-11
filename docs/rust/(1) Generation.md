# Generation (1)

## The Problem
Let's first of all understand the problem that we have tried to solve, because once you understand this, you will understand the reasoning behind why we have implemented the system in this manner.

We have two main entities in our system: **businesses** and **people**. These are the most significant parts of the economy and the game because they are solely responsible for the health of the economy. 

### Businesses
Businesses are dynamic. They scale and change based on the demand for a specific product type. The demand influences factors such as stock, product cost, employee count. These in turn influence the expected profits for the month.

### People
People have a demand which is generated based on factors such as their salary and welfare. If their demand is not met then their welfare is decreased, which as a result increases their demand.

### You see..

Do you see the problem here? Businesses rely on people - specifically their demand for a product. And people rely on businesses - which determines their welfare and their demand.

## The solution (important part)