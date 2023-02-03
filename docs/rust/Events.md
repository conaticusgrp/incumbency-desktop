# Frontend Events Documentation

## Rule IDs
Tax - `0` \
Business Tax - `1` \
Business Funding - `2` \
Healthcare Deny Age - `3` \
Deny Health Percentage - `4` \
Cover Food - `5` \
Cover Food Unemployed - `6`

## App IDs
Finance - `2` \
Healthcare - `3` \
Welfare - `4` \
Business - `5`



## send (`app_open`)
### Description
Sent when an app is opened/maximised.

### Payloads

Input Payload:
```ts
{
    app_id: number
}
```

Output Payload: `String` - stringified JSON data for the app



## send (`app_close`)
### Description
Sent when an app is closed/minimised.

### Payloads

Input Payload:
```ts
{
    app_id: number
}
```

Output Payload: None



## send (`enable_rule`)
### Description
Sent when a game rule is enabled.

### Payloads

Input Payload:
```ts
{
    rule_id: number
}
```

Output Payload: None



## send (`disable_rule`)
### Description
Sent when a game rule is disabled.

### Payloads

Input Payload:
```ts
{
    rule_id: number
}
```

Output Payload: None


## send (`update_rule`)
### Description
Sent when a game rule is updated.

Input Payload:
```ts
    rule_id: number;
    data: object,
```

### `payload.data` Payloads

Tax Rule:
```ts
{
    minimum_salary: number,
    tax_rate: number,
}
```
\
\
Business Tax Rule:
```ts
{
    minimum_monthly_income: number,
    tax_rate: number,
}
```
\
\
Business Funding Rule:
```ts
{
    fund: number,
    maximum_income: number,
    business_count: number,
}
```

Outputs:
```ts
{
    budget_cost: number,
}
```

```ts
{
    error: string,
}
```
\
\
Healthcare Deny Age Rule:
```ts
{
    maximum_age: number,
}
```
\
\
Deny Health Percentage Rule:
```ts
{
    maximum_percentage: number,
}
```
\
\
Cover Food Cost Rule:
```ts
{
    people_count: number,
    maximum_salary: number,
}
```
Outputs:
```ts
{
    budget_cost: number,
}
```

```ts
{
    error: string,
}
```
\
\
Cover Food Cost Unemployed Rule:
```ts
{
    people_count: number,
}
```

Outputs:
```ts
{
    budget_cost: number,
}
```

```ts
{
    error: string,
}
```
\
\
Default Output: `{}`


## send (`update_tax_rate`)
### Description
Sent when the player updates the standard tax rate.

### Payloads

Input Payload:
```ts
{
    tax_rate: number
}
```

Output Payload: `number` - The expected income from people with this tax rate


## send (`update_business_tax_rate`)
### Description
Sent when the player updates the business tax rate.

### Payloads

Input Payload:
```ts
{
    tax_rate: number
}
```

Output Payload: `number` - The expected income from businesses with this tax rate


## send (`update_healthcare_budget`)
### Description
Sent when the player updates the total healthcare budget.

### Payloads

Input Payload:
```ts
{
    new_budget: number
}
```

Output Payloads:
```ts
{
    used_hospital_capacity: number,
    total_hospital_capacity: number,
}
```
```ts
{
    error: string,
}
```


## send (`update_welfare_budget`)
### Description
Sent when the player updates the welfare budget.

### Payloads

Input Payload:
```ts
{
    new_budget: number
}
```

Output Payloads:
```ts
{}
```
```ts
{
    error: string,
}
```


## send (`update_business_budget`)
### Description
Sent when the player updates the business budget.

### Payloads

Input Payload:
```ts
{
    new_budget: number
}
```

Output Payloads:
```ts
{}
```
```ts
{
    error: string,
}
```

## send (`update_childcare_capacity`)
### Description
Sent when the player updates the total childcare capacity.

### Payloads

Input Payload:
```ts
{
    new_capacity: number
}
```

Output Payloads:
```ts
{}
```
```ts
{
    error: string,
}
```

## send (`update_adultcare_capacity`)
### Description
Sent when the player updates the total adultcare capacity.

### Payloads

Input Payload:
```ts
{
    new_capacity: number
}
```

Output Payloads:
```ts
{}
```
```ts
{
    error: string,
}
```


## send (`update_eldercare_capacity`)
### Description
Sent when the player updates the total eldercare capacity.

### Payloads

Input Payload:
```ts
{
    new_capacity: number
}
```

Output Payloads:
```ts
{}
```
```ts
{
    error: string,
}
```

## recv (`update_app`)
### Description
Received when an app needs a data update.

Output Payload: `object` - Object containing the updated app data (this will have the same data keys as sent back in the `app_opened` event)