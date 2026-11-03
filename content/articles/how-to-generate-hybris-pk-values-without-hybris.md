# How to generate hybris PK values without hybris

The logic of the PK is a bit complex. More than one implementation of
the PK exists: a legacy one, based on UUIDs, and another based on a
counter, which is the one I'm going to explain.

The structure of the PK is the following:

| Bits | Content |
|---|---|
| 63–44 | counter, high bits |
| 43 | marker: 1 when the PK is counter based, 0 when it is UUID based |
| 42–15 | counter, low bits |
| 14–0 | type code |

As you see:

- one bit, bit 43, tells whether the PK is based on a counter or on a
  UUID. The UUID version is legacy and deprecated; by default hybris
  uses the counter-based one.
- the low bits (0–14) hold the type code, which is the deployment code
  associated with the type that stores your object. This is why from a
  PK we can know which type it is, and how the PK Analyzer in the HAC
  identifies the type of a PK.

> **Note:** the PK depends:
>
> - on how many times the counter has been incremented in one system,
>   and
> - on the type code (deployment code) associated with a type in a
>   specific system.
>
> This means that you cannot copy a PK value from one system to another
> without thinking about what you are doing.

## What is the type code

Also known as deployment code, it is a number associated with the
actual table that stores the objects of your hybris type. You can find
it in the HAC, or you can retrieve it with the following Groovy:

```groovy
String type = 'Product'
String jndiName = Registry.getPersistenceManager().getJNDIName(type);
return Registry.getPersistenceManager().getItemDeployment(jndiName).getTypeCode();
```

## Where is the counter?

The counter is saved in a database table, `numberseries`:

```sql
describe numberseries;
```

| Field | Type | Null | Key | Default | Description |
|---|---|---|---|---|---|
| hjmpTS | bigint(20) | YES | | | Version in the optimistic lock scenario; not relevant in this discussion. |
| serieskey | varchar(255) | NO | PRI | | For the scope of this discussion, the deployment code prefixed by `pk_`. |
| seriestype | int(11) | YES | | 0 | Always 0 or 1; for our scope it is always 1, NUMERIC. |
| currentValue | bigint(20) | YES | | | The value to use for the next PK. |
| template | varchar(255) | YES | | | Normally not used; for our needs you can ignore it. |

But hybris does not read and write the value of the PK in the database
for each item it has to create: the value in the database always
indicates the next one available.

So let's say that a hybris instance reads the counter value 1001 from
the database. It will save the next one as 1001 + 1000 = 2001, so it
has 1000 PKs free to be used without the need to call the database.

The cache pool is controlled by the configuration property
`numberseries.cache.size`, which is 1000 by default. Be careful: it is
also configurable by type code, with a configuration property like
`numberseries.cache.size.pk_<deployment_code>`. It does something
similar to this:

```java
protected int getCacheSize(String key) {
    // get config of "numberseries.cache.size." + key or -1
    // if -1 get config of "numberseries.cache.size" or 1000
    // return max(last config value, 1)
}
```

This class, `DefaultCachingSerialNumberGenerator`, also has a method to
reset the counter:

```java
public void resetSeries(String key, final int type, final long startValue) {
    // ...
}
```

So, to calculate some PK values and use them without creating issues,
you first need to read the current value of the counter:

```sql
select currentValue from numberseries where serieskey='pk_1055';
```

then reset the counter as hybris does, creating space for your
generated PKs:

```groovy
Registry.getCurrentTenant().getSerialNumberGenerator().resetSeries('pk_1055', 1055, counterValue + 10000000)
```

where `counterValue` is the value just read, and 1055 is the
deployment code of your type. The counter values from `counterValue`
up to `counterValue + 10000000`, excluded, are now yours.

Then, given a value of the counter, we can use the following Groovy to
get our PK value:

```groovy
def calculatePK(counter){
    long counterAndTypeCode = counter << 15 | 1055
    long upperWithoutBit43 = (counterAndTypeCode & -8796093022208L) << 1
    long lowerWithoutBit43 = counterAndTypeCode & 8796093022207L
    long rawPKBits = upperWithoutBit43 | 8796093022208L | lowerWithoutBit43
    return rawPKBits;
}
```

On SAP's community site, the answer
[PK generation outside hybris](https://answers.sap.com/questions/12765980/pk-generation-outside-hybris.html)
implements the same with an Oracle stored procedure.

The subject is also covered by
[SAP's official documentation, version 2105](https://help.sap.com/viewer/d0224eca81e249cb821f2cdf45a82ace/2105/en-US/8c2d4abd866910149207b2f33deaf652.html).

---

The interfaces, classes and configuration quoted from SAP Commerce
belong to SAP; the MIT-0 licence covers only my own code samples.
