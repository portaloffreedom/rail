import QtQuick 2.2
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.0

ApplicationWindow {
  visible: true
  title: "Factorial"

  property int margin: 11
  width: mainLayout.implicitWidth + 2 * margin
  height: mainLayout.implicitHeight + 2 * margin
  minimumWidth: mainLayout.Layout.minimumWidth + 2 * margin
  minimumHeight: mainLayout.Layout.minimumHeight + 2 * margin

  ColumnLayout {
    id: mainLayout
    anchors.fill: parent
    anchors.margins: margin

    RowLayout {
      ColumnLayout {
        TextField {
          id: usernameField
          Layout.fillWidth: true

          placeholderText: "Username"
          focus: true

          onAccepted: doTestConnect()
        }
        TextField {
          id: passwordField
          Layout.fillWidth: true

          placeholderText: "Password"

          onAccepted: doTestConnect()
        }
      }

      Button {
        text: "TestConnect"

        onClicked: doTestConnect()
      }
    }

    TextArea {
      id: resultArea
      Layout.fillWidth: true
      Layout.fillHeight: true
    }
  }

  function doTestConnect() {
    var username = usernameField.text;
    var password = passwordField.text;
    resultArea.text = MailClient.test_connect(username, password);
  }

/*
  Connections {
    target: factorial
    onTest: console.log("Got test signal!")
  }
*/
}