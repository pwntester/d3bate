import React from "react";
import { Text, TouchableOpacity, View } from "react-native";
import { Ionicons } from "@expo/vector-icons";
import { colours } from "../styles";
import { connect } from "react-redux";
import { Link } from "../routing/routing";

const linkStyles = {
    fontSize: 24,
    color: "white",
    paddingLeft: 10,
    paddingTop: 2
};

class Navbar extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            open: false
        }
    }

    render() {
        if (this.state.open)
            return <>
                <View
                    style={{ backgroundColor: colours.primary, display: "flex", flexDirection: "row", padding: 10 }}>
                    <TouchableOpacity style={{ display: "flex", flexDirection: "row" }} onPress={press => {
                        this.setState({ open: false })
                    }}>
                        <Ionicons name="md-close" size={32} color="white" />
                        <Text style={linkStyles}>close</Text>
                    </TouchableOpacity>

                </View>
                <View style={{ backgroundColor: colours.primary, display: "flex", flexDirection: "column", padding: 10 }}>
                    <Link to="/" style={{ textDecoration: 'none' }}><Text
                        style={linkStyles}>Home</Text></Link>
                    {this.props.auth.jwt ? <>
                        <Link to="/club" style={{ textDecoration: 'none' }}><Text
                            style={linkStyles}>Club</Text></Link>
                        <Text style={linkStyles}>Calendar</Text>
                        <Text style={linkStyles}>Join debate</Text>
                    </> :
                        <>
                            <Link to="/login" style={{ textDecoration: 'none' }}><Text
                                style={linkStyles}>Login</Text></Link>
                            <Link to="/register" style={{ textDecoration: 'none' }}><Text
                                style={linkStyles}>Register</Text></Link>
                        </>}
                </View>
            </>;
        else
            return (
                <View style={{ backgroundColor: colours.primary, display: "flex", flexDirection: "row", padding: 10 }}>
                    <TouchableOpacity onPress={press => {
                        this.setState({ open: true })
                    }}>
                        <Ionicons name="md-menu" size={32} color="white" />
                    </TouchableOpacity>
                    <Text style={linkStyles}>d3bate</Text>
                </View>
            );
    }
}

export default connect((state, ownProps) => {
    return {
        auth: state.auth,
        ...ownProps
    }
})(Navbar);
