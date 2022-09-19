Feature: User create a item

  Scenario: User want to create a item
    Given I have logged in
    When I click on the Items button
    Then I should have see my items, a text field and A button named Create new Item
    When I fill in text field with item name and click to Create new Item
    Then I should have redirct to items page and see new item in my items list
