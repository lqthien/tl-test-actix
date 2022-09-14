Feature: User registration

  Scenario: User want to view their items
    Given I have logged in
    When I click on the Items button
    Then I should see all of my items
       