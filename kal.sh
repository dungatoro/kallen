#!/bin/sh

ACTION=$(gum choose "Show schedule" "Add an event" "Remove an event" "Update an event")

if [ "$ACTION" = "Show schedule" ]
then
    OPTION=$(gum choose "today" "this week" "given day" "week beginning..")

    if [ "$OPTION" = "today" ]
    then
        kallen day
    elif [ "$OPTION" = "this week" ]
    then
        kallen week
    else
        DATE=$(gum input --placeholder "DD/MM/YYYY")
        if [ "$OPTION" = "given day" ]
        then
            kallen day "$DATE"
        else
            kallen week "$DATE"
        fi
    fi
elif [ "$ACTION" = "Add an event" ]
then
    DATE=$(gum input --placeholder "DD/MM/YYYY")
    TIME=$(gum input --placeholder "20:04")
    DESC=$(gum input --placeholder "really interesting description")
    kallen add --desc "$DESC" --date "$DATE" -t "$TIME"
    kallen day --date "$DATE"
elif [ "$ACTION" = "Remove an event" ]
then
    DATE=$(gum input --placeholder "DD/MM/YYYY")
    kallen day --date "$DATE"
    echo "Choose event: "
    IDX=$(gum input --placeholder "8")
    kallen del --date "$DATE" -i "$IDX"
    kallen day --date "$DATE"
else 
    DATE=$(gum input --placeholder "DD/MM/YYYY")
    kallen day --date "$DATE"
    echo "Choose event: "
    IDX=$(gum input --placeholder "8")
    TIME=$(gum input --placeholder "20:04")
    DESC=$(gum input --placeholder "really interesting description")
    kallen update --date "$DATE" -i "$IDX" --desc "$DESC" -t "$TIME"
    kallen day --date "$DATE"
fi


