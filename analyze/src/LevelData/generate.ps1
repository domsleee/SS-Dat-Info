$folder = "/Users/dom/Applications/supreme snowboarding.app/Contents/drive_c/Games/Supreme"

$names = @{
    "AlpineEasy" = "AlpineEasy"
    "AlpineMedium" = "AlpineMedium"
    "AlpineHard" = "AlpineHard"
    "ForestEasy" = "ForestEasy"
    "ForestMedium" = "ForestMedium"
    "ForestHard" = "ForestHard"
    "VillageEasy" = "VillageEasy"
    "VillageMedium" = "VillageMedium"
    "VillageHard" = "VillageHard"
}

foreach ($level in @("Alpine", "Village", "Forest")) {
    foreach ($difficulty in @("Easy", "Medium", "Hard")) {
        $levelName = $names[$level + $difficulty]
        bun start dump-objects "$folder/Data/Levels/$level/Tracks/$difficulty/Object_Data.txt" "Check_Point_1" "Check_Point_2" "Start_Point" "Player_Start_Location" "Finish_Point" > ./Raw/$levelName.txt
    }
}