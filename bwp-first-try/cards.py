import csv

guests = list(csv.DictReader(open("cards.csv")))

for guest in guests:
    guest_id = guest["Guest"]

    commons = {i: 0 for i in range(3)}

    # Compare against all possible targets
    for target in guests:
        if guest_id == target["Guest"]:
            continue

        guest_cards = {guest[col] for col in ["Group 1", "Group 2", "Group 3", "Group 4"]}
        target_cards = {target[col] for col in ["Group 1", "Group 2", "Group 3", "Group 4"]}
        in_common = len(guest_cards & target_cards)
        if in_common > 2:
            raise Exception(f"Conflict with {guest_id} and {target['Guest']}: {in_common}")
        commons[in_common] += 1

    # Print results
    print(f"Guest {guest_id}: {commons[0]} share none, {commons[1]} share 1, {commons[2]} share 2")
