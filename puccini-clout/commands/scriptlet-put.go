package commands

import (
	"github.com/spf13/cobra"
	formatpkg "github.com/tliron/kutil/format"
	"github.com/tliron/kutil/terminal"
	urlpkg "github.com/tliron/kutil/url"
	"github.com/tliron/kutil/util"
	cloutpkg "github.com/tliron/puccini/clout"
	"github.com/tliron/puccini/clout/js"
)

func init() {
	scriptletCommand.AddCommand(putCommand)
	putCommand.Flags().StringVarP(&output, "output", "o", "", "output Clout to file (default is stdout)")
}

var putCommand = &cobra.Command{
	Use:   "put [NAME] [JavaScript PATH or URL] [[Clout PATH or URL]]",
	Short: "Put JavaScript scriptlet in Clout",
	Long:  ``,
	Args:  cobra.RangeArgs(2, 3),
	Run: func(cmd *cobra.Command, args []string) {
		scriptletName := args[0]
		scriptletUrl := args[1]

		var url string
		if len(args) == 3 {
			url = args[2]
		}

		clout, err := cloutpkg.Load(url, inputFormat)
		util.FailOnError(err)

		urlContext := urlpkg.NewContext()
		defer urlContext.Release()

		scriptletUrl_, err := urlpkg.NewValidURL(scriptletUrl, nil, urlContext)
		util.FailOnError(err)

		scriptlet, err := urlpkg.ReadString(scriptletUrl_)
		util.FailOnError(err)

		err = js.SetScriptlet(scriptletName, js.CleanupScriptlet(scriptlet), clout)
		util.FailOnError(err)

		err = formatpkg.WriteOrPrint(clout, format, terminal.Stdout, strict, pretty, output)
		util.FailOnError(err)
	},
}
