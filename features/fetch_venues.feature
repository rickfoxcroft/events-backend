Feature: Fetch Venue List
  As an event organizer
  I want to view a list of all available venues
  So that I can choose the best one for my event

  Scenario: Viewing an empty list of venues
    Given there are no venues listed on the platform
    When I request the list of all venues
    Then I should receive an empty list of venues

  Scenario: Viewing a list of available venues
    Given the following venues exist:
      | Name           | Location     | Capacity |
      | Grand Ballroom | Downtown     |      500 |
      | Tech Hub       | North Sector |      100 |
    When I request the list of all venues
    Then I should see 2 venues in the list
    And I should see "Grand Ballroom" in the list of venues
    And I should see "Tech Hub" in the list of venues
