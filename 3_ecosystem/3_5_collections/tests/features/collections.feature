Feature: Immutable Collections Feature
    Scenario: Mutate cloned List
        Given A List of followings users
        When The List is copied and modified
        Then The original List remains the same
    Scenario: Find a user in List by name
        Given A List with "Jack, John, Doe"
        Then The searched_name is in the List
            |searched_name|
            |Jack|
            |John|
            |Doe|
