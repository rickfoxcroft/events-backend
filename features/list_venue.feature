Feature: Venue Listing
  As a venue owner
  I want to list my venue on the platform
  So that potential event organizers can find and book it

  Scenario: Successfully listing a new venue
    Given I am a registered venue owner
    When I submit the following details for my new venue:
      | Name             | Location      | Capacity |
      | Grand Ballroom   | Downtown      | 500      |
    Then my venue should be successfully listed
    And I should see "Grand Ballroom" in my list of venues
