using date.(now, Date)
using time.Time

-- denoting private fields with - and public ones with +
-- this will automatically derive (pub function name(Person) -> String | function birthday(Person) -> Date | ... | set_hair &Person HairColor)
type Person
- name String
- birthday Date
- gender Gender
+ mut hair HairColor

type Gender
| Female
| Male
| Other

type HairColor
| Blond
| Brown
| Black
| Red

pub function age(p Person) -> Time = p:birthday - now

pub function sample_person() -> Person = Person name="Peter" birthday = (date.ymd 2003 03 17) gender=Gender.Male hair=HairColor.Red
