[
  {
    $match: {
      editingState: "Ready"
    }
  },
  {
    $lookup: {
      from: "actionTypes",
      localField: "action.typeId",
      foreignField: "_id",
      as: "action.typeObj"
    }
  },
  {
    $unwind: "$action.typeObj"
  },
  {
    $lookup: {
      from: "trainingCosts",
      localField: "trainingCostId",
      foreignField: "_id",
      as: "trainingCostObj"
    }
  },
  {
    $unwind: "$trainingCostObj"
  },
  {
    $sort: {
      tier: 1,
      catagoryOrder: 1,
      "trainingCostObj.order": 1,
      "action.manaCost": 1,
      title: 1
    }
  },
  {
    $set: {
      trainingCost: "$trainingCostObj.title",
      "action.type": "$action.typeObj.title"
    }
  },
  {
    $unset: [
      "Id",
      "editingState",
      "action.typeId",
      "action.typeObj",
      "trainingCostId",
      "trainingCostObj",
      "catagoryOrder"
    ]
  }
]