import {action, decorate, observable, observe} from "mobx";
import {auth, firebase} from "../sync";

let clubMembershipsSnapshot;

let findItem = (item, list) => {
    return list.findIndex(o => {
        return o.id === item.id
    });
};

class Messages {
    messages = [];

    addMessage(message) {
        this.messages.push({
            id: this.messages.length - 1,
            title: message.title,
            body: message.body,
            category: message.category
        })
    }

    deleteMessage(id) {
        let message = this.messages.findIndex(o => {
            return o.id === id
        });
        this.messages.splice(message, 1);
    }
}

decorate(Messages, {
    messages: observable,
    addMessage: action,
    deleteMessage: action
});

export const messages = new Messages();

class Calendar {
    events = [];
    populated = false;

    updateEvent(event) {
        let foundEvent = findItem(event, this.events);

        if (foundEvent !== -1) {
            this.events[foundEvent] = event
        }
        else {
            this.events.push(event)
        }
    }
}


decorate(Calendar, {
    events: observable,
    populated: observable,
    updateEvents: action
});

export const calendar = new Calendar();


class DebatingClubs {
    clubs = [];
    populated = false;

    updateClub(club) {
        let foundClub = findItem(club, this.clubs);
        if (foundClub !== -1) {
            this.clubs[foundClub] = club
        }
        else {
            this.clubs.push(club)
        }
    }
}

decorate(DebatingClubs, {
    clubs: observable,
    populated: observable,
    updateClub: action
});

export const debatingClubs = new DebatingClubs();


class AttendanceEvents {
    events = [];
    populated = false;

    updateEvent(event) {
        let foundEvent = findItem(event, this.events);
        if (foundEvent !== -1) {
            this.events[foundEvent] = event
        }
        else {
            this.events.push(event)
        }
    }
}

decorate(AttendanceEvents, {
        events: observable,
        populated: observable,
        updateEvent: action
    }
);

export const attendanceEvents = new AttendanceEvents();

class AppState {
    user = null;
    userDocument = null;

    setUser(user) {
        this.user = user
    }

    setUserDocument(document) {
        this.userDocument = document
    }
}

decorate(AppState, {
    user: observable,
    userDocument: observable,
    setUser: action,
    setUserDocument: action,
});

export const appState = new AppState();

observe(appState, "user", change => {
    let uObject = change.newValue;
    if (uObject) {
        let userSnapshot = firebase.firestore().collection('users').doc(uObject.uid).get()
            .then(result => {
                appState.setUserDocument(result)
            });
        let clubMembershipsSnapshot = firebase.firestore().collection('clubMemberships').where('userID', '==', uObject.uid)
            .onSnapshot(result => {
                if (result.size > 0) {
                    result.docs.forEach(doc => {
                        debatingClubs.updateClub({id: doc.id, ...doc.data()});
                        firebase.firestore().collection('calendar').where('clubID', '==', doc.id)
                            .onSnapshot((snapshot => {
                                snapshot.forEach(
                                    doc => {
                                        calendar.updateEvent({id: doc.id, ...doc.data()})
                                    }
                                )
                            }));
                        firebase.firestore().collection('attendance').where('userID', '==', uObject.uid)
                            .onSnapshot(snapshot => {
                                snapshot.forEach(doc => {
                                    attendanceEvents.updateEvent({id: doc.id, ...doc.data()})
                                })
                            })
                    });
                }
            });
    }
    else {
        appState.setUserDocument(null);
    }
});

auth.onAuthStateChanged(uObject => {
    appState.setUser(uObject);
});
