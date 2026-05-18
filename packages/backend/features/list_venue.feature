Feature: Venue Listing
  As a venue owner
  I want to list my venue on the platform
  So that users can discover and book it

  Scenario: Successfully listing a new venue
    Given I am a registered venue owner
    When I submit the following details for my new venue:
      | Name           | Location | Capacity |
      | Grand Ballroom | Downtown | 500      |
    Then my venue should be successfully listed
    And I should see "Grand Ballroom" in my list of venues

  Scenario: Listing a venue with images
    Given I am a registered venue owner
    When I upload the following images:
      | filename    | type       |
      | lounge1.jpg | image/jpeg |
      | lounge2.png | image/png  |
    And I submit the following details for my new venue:
      | Name       | Location | Capacity |
      | Sky Lounge | Midtown  | 150      |
    Then my venue should be successfully listed
    And the venue "Sky Lounge" should have 2 images attached
