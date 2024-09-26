// main skill cleanup
db["skills"].updateMany( {},
  {
    "$rename": { 
      "Title": "title",
      "Description": "description",
      "Tier.Title": "tier",
      "State.Title": "editingState",
      "Flow.Id": "catagoryOrder"
    }
  }
)
db["skills"].updateMany(
  { "ManaCost":{ "$exists": true } },
  {
    "$set": {
      "catagoryOrder": 3
    },
    "$unset": {
      "action.cost": ""
    },
    "$rename": {
      "ManaCost": "action.manaCost"
    }
  }
)
db["skills"].updateMany(
  {}, { "$unset": { "IsSpell": "", "Paths": "", "Keywords": "", "Type": "", "Tier": "", "State": "", "Flow": "" } }
)
db["skills"].updateMany(
  { "catagoryOrder":{ "$exists": false } },
  {
    "$set": { "catagoryOrder": 0 }
  }
)


// secondary table renames
db["keywords"].updateMany(
  {}, { "$rename": { "Title": "title" } }
)
db["trainingCosts"].updateMany(
  {}, { "$rename": { "Title": "title", "Order": "order" } }
)
db["paths"].updateMany(
  {}, { "$rename": { "Title": "title", "Order": "order" } }
)
db["resources"].updateMany(
  {}, { "$rename": { "Title": "title", "Die": "die" } }
)
db["actionTypes"].updateMany(
  {}, { "$rename": { "Title": "title" } }
)

// remove null fields
db["skills"].updateMany(
  { "ManaCost":{"$exists": true, "$eq": null} },
  {
    "$unset": { "ManaCost": "", }
  }
)
db["skills"].updateMany(
  { "LimitType":{"$exists": true, "$eq": null} },
  {
    "$unset": { "LimitType": "", }
  }
)


// update training cost ids
db["skills"].updateMany(
  { "Cost.Title": "Inherient" },
  {
    "$unset": { "Cost": "" },
    "$set": { "trainingCostId": ObjectId( "66f1eba07e098103fc421a43" ) }
  }
)
db["skills"].updateMany(
  { "Cost.Title": "Full" },
  { 
    "$unset": { "Cost": "" },
    "$set": { "trainingCostId": ObjectId( "66f1eba07e098103fc421a44" ) }
  }
)
db["skills"].updateMany(
  { "Cost.Title": "Half" }, 
  { 
    "$unset": { "Cost": "" },
    "$set": { "trainingCostId": ObjectId( "66f1eba07e098103fc421a45" ) }
  }
)
db["skills"].updateMany(
  { "Cost.Title": "Keystone" },
  {
    "$unset": { "Cost": "" },
    "$set": { "trainingCostId": ObjectId( "66f1eba07e098103fc421a46" ) }
  }
)
db["skills"].updateMany(
  { "Cost.Title": "Cantrip" },
  {
    "$unset": { "Cost": "" },
    "$set": { "trainingCostId": ObjectId( "66f1eba07e098103fc421a47" ) }
  }
)
db["skills"].updateMany(
  { "Cost.Title": "Spell" },
  {
    "$unset": { "Cost": "" },
    "$set": { "trainingCostId": ObjectId( "66f1eba07e098103fc421a48" ) }
  }
)


db["skills"].updateMany(
  { "action.typeId.$oid": "66f319c27e098103fc421a5d" },
  { "$set": { "action.typeId": ObjectId("66f319c27e098103fc421a5d") } }
)
db["skills"].updateMany(
  { "action.typeId.$oid": "66f319c27e098103fc421a5e" },
  { "$set": { "action.typeId": ObjectId("66f319c27e098103fc421a5e") } }
)
db["skills"].updateMany(
  { "action.typeId.$oid": "66f319c27e098103fc421a5f" },
  { "$set": { "action.typeId": ObjectId("66f319c27e098103fc421a5f") } }
)
db["skills"].updateMany(
  { "action.typeId.$oid": "66f319c27e098103fc421a60" },
  { "$set": { "action.typeId": ObjectId("66f319c27e098103fc421a60") } }
)
db["skills"].updateMany(
  { "action.typeId.$oid": "66f319c27e098103fc421a61" },
  { "$set": { "action.typeId": ObjectId("66f319c27e098103fc421a61") } }
)
db["skills"].updateMany(
  { "action.typeId.$oid": "66f319c27e098103fc421a62" },
  { "$set": { "action.typeId": ObjectId("66f319c27e098103fc421a62") } }
)
db["skills"].updateMany(
  { "action.typeId.$oid": "66f319c27e098103fc421a63" },
  { "$set": { "action.typeId": ObjectId("66f319c27e098103fc421a63") } }
)
db["skills"].updateMany(
  { "action.typeId.$oid": "66f319c27e098103fc421a64" },
  { "$set": { "action.typeId": ObjectId("66f319c27e098103fc421a64") } }
)
db["skills"].updateMany(
  { "action.typeId.$oid": "66f319c27e098103fc421a65" },
  { "$set": { "action.typeId": ObjectId("66f319c27e098103fc421a65") } }
)


// update action types
db["skills"].updateMany(
  { "BaseType.Title": "Boon" },
  {
    "$unset": { "BaseType": "" },
    "$set": { "action.typeId": ObjectId( "66f319c27e098103fc421a5d" ) }
  }
)
db["skills"].updateMany(
  { "BaseType.Title": "Action" },
  {
    "$unset": { "BaseType": "" },
    "$set": { "action.typeId": ObjectId( "66f319c27e098103fc421a5e" ) }
  }
)
db["skills"].updateMany(
  { "BaseType.Title": "Interaction" },
  {
    "$unset": { "BaseType": "" },
    "$set": { "action.typeId": ObjectId( "66f319c27e098103fc421a5f" ) }
  }
)
db["skills"].updateMany(
  { "BaseType.Title": "Reaction" },
  {
    "$unset": { "BaseType": "" },
    "$set": { "action.typeId": ObjectId( "66f319c27e098103fc421a60" ) }
  }
)
db["skills"].updateMany(
  { "BaseType.Title": "Reflex" },
  {
    "$unset": { "BaseType": "" },
    "$set": { "action.typeId": ObjectId( "66f319c27e098103fc421a61" ) }
  }
)
db["skills"].updateMany(
  { "BaseType.Title": "Trigger" },
  {
    "$unset": { "BaseType": "" },
    "$set": { "action.typeId": ObjectId( "66f319c27e098103fc421a62" ) }
  }
)
db["skills"].updateMany(
  { "BaseType.Title": "Complex Action" },
  {
    "$unset": { "BaseType": "" },
    "$set": { "action.typeId": ObjectId( "66f319c27e098103fc421a63" ) }
  }
)
db["skills"].updateMany(
  { "BaseType.Title": "Extended Action" },
  {
    "$unset": { "BaseType": "" },
    "$set": { "action.typeId": ObjectId( "66f319c27e098103fc421a64" ) }
  }
)
db["skills"].updateMany(
  { "BaseType.Title": "Free Action" },
  {
    "$unset": { "BaseType": "" },
    "$set": { "action.typeId": ObjectId( "66f319c27e098103fc421a65" ) }
  }
)

// update action modifiers
db["skills"].updateMany(
  { "LimitType.Title": "Initial" },
  {
    "$unset": { "LimitType": "" },
    "$set": { "action.initial": true }
  }
)


db["skills"].updateMany(
  { "trainingCostId.$oid": "66f1eba07e098103fc421a43" },
  { "$set": { "trainingCostId": ObjectId("66f1eba07e098103fc421a43") } }
)


